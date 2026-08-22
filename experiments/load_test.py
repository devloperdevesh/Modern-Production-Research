from locust import HttpUser, between, task


class MPRLoadUser(HttpUser):
    wait_time = between(0.1, 0.3)

    @task
    def health_check(self):
        self.client.get("/health")
