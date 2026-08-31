from __future__ import annotations

import argparse
import csv
import os
import re
from dataclasses import dataclass
from pathlib import Path

from github import Github
from github.Issue import Issue


DEFAULT_REPOSITORIES = (
    "tiangolo/fastapi",
    "langchain-ai/langchain",
    "crewAIInc/crewAI",
)

NETWORK_TERMS = (
    "network",
    "connection",
    "connect",
    "timeout",
    "socket",
    "http",
    "tcp",
    "latency",
    "throughput",
    "overload",
    "concurrency",
    "rate limit",
    "rate-limit",
    "backpressure",
    "deadlock",
    "retry",
    "connection pool",
)

PERFORMANCE_TERMS = (
    "slow",
    "latency",
    "performance",
    "throughput",
    "timeout",
    "overload",
    "memory",
    "cpu",
    "concurrency",
    "queue",
    "blocking",
    "saturation",
)

PRIMITIVE_MAP = {
    "backpressure": "Backpressure / QueueGuard",
    "overload": "Adaptive load shedding",
    "saturation": "Adaptive load shedding",
    "rate limit": "Token bucket",
    "rate-limit": "Token bucket",
    "concurrency": "Semaphore / bounded concurrency",
    "timeout": "Timeout policy",
    "retry": "Retry budget",
    "latency": "Runtime / telemetry path",
    "throughput": "Runtime / admission control",
    "connection pool": "Bounded concurrency",
    "deadlock": "Concurrency primitives",
    "queue": "Backpressure",
}


@dataclass(frozen=True)
class Prospect:
    repository: str
    issue_number: int
    title: str
    url: str
    score: int
    category: str
    mpr_primitive: str


def issue_text(issue: Issue) -> str:
    return f"{issue.title}\n{issue.body or ''}".lower()


def contains_any(text: str, terms: tuple[str, ...]) -> bool:
    return any(term in text for term in terms)


def classify(text: str) -> str:
    if contains_any(text, ("timeout", "connection", "socket", "network")):
        return "network/reliability"

    if contains_any(text, ("latency", "slow", "throughput", "performance")):
        return "performance"

    if contains_any(text, ("overload", "saturation", "queue", "backpressure")):
        return "admission-control"

    if contains_any(text, ("concurrency", "deadlock", "blocking")):
        return "concurrency"

    if contains_any(text, ("retry", "rate limit", "rate-limit")):
        return "resilience/rate-limit"

    return "other"


def primitive_for(text: str) -> str:
    matches = [
        primitive
        for term, primitive in PRIMITIVE_MAP.items()
        if term in text
    ]

    if not matches:
        return "Needs manual review"

    return " / ".join(dict.fromkeys(matches))


def score_issue(text: str) -> int:
    score = 0

    if contains_any(text, NETWORK_TERMS):
        score += 3

    if contains_any(text, PERFORMANCE_TERMS):
        score += 3

    if "production" in text:
        score += 2

    if "timeout" in text or "latency" in text:
        score += 2

    if "concurrency" in text or "overload" in text:
        score += 2

    return score


def discover(
    github: Github,
    repositories: tuple[str, ...],
    limit: int,
) -> list[Prospect]:
    prospects: list[Prospect] = []

    for repository_name in repositories:
        repository = github.get_repo(repository_name)

        issues = repository.get_issues(
            state="open",
            sort="updated",
            direction="desc",
        )

        scanned = 0

        for issue in issues:
            if issue.pull_request is not None:
                continue

            text = issue_text(issue)

            if not contains_any(text, NETWORK_TERMS + PERFORMANCE_TERMS):
                continue

            score = score_issue(text)

            if score < 5:
                continue

            prospects.append(
                Prospect(
                    repository=repository_name,
                    issue_number=issue.number,
                    title=re.sub(r"\s+", " ", issue.title).strip(),
                    url=issue.html_url,
                    score=score,
                    category=classify(text),
                    mpr_primitive=primitive_for(text),
                )
            )

            scanned += 1

            if scanned >= limit:
                break

    return sorted(
        prospects,
        key=lambda prospect: (
            -prospect.score,
            prospect.repository,
            prospect.issue_number,
        ),
    )


def write_csv(prospects: list[Prospect], output: Path) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)

    with output.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)

        writer.writerow(
            (
                "repository",
                "issue_number",
                "title",
                "url",
                "score",
                "category",
                "mpr_primitive",
            )
        )

        for prospect in prospects:
            writer.writerow(
                (
                    prospect.repository,
                    prospect.issue_number,
                    prospect.title,
                    prospect.url,
                    prospect.score,
                    prospect.category,
                    prospect.mpr_primitive,
                )
            )


def main() -> None:
    parser = argparse.ArgumentParser(
        description=(
            "Discover public GitHub issues relevant to "
            "MPR performance primitives."
        )
    )

    parser.add_argument(
        "--repo",
        action="append",
        dest="repositories",
        help="Repository in owner/name format. Can be supplied multiple times.",
    )

    parser.add_argument(
        "--limit",
        type=int,
        default=25,
        help="Maximum qualifying issues per repository.",
    )

    parser.add_argument(
        "--output",
        type=Path,
        default=Path("artifacts/traction/prospects.csv"),
    )

    args = parser.parse_args()

    token = os.environ.get("GITHUB_TOKEN")

    if not token:
        raise SystemExit(
            "GITHUB_TOKEN is required. Set a GitHub token "
            "with read-only public repository access."
        )

    repositories = tuple(args.repositories or DEFAULT_REPOSITORIES)

    github = Github(token)

    prospects = discover(
        github=github,
        repositories=repositories,
        limit=args.limit,
    )

    write_csv(prospects, args.output)

    print(f"Qualified prospects: {len(prospects)}")
    print(f"Output: {args.output}")

    for prospect in prospects[:10]:
        print(
            f"[{prospect.score}] "
            f"{prospect.repository}#{prospect.issue_number} "
            f"{prospect.category} -> {prospect.mpr_primitive}"
        )


if __name__ == "__main__":
    main()
