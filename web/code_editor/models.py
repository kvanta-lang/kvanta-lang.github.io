import uuid

from django.contrib.auth.models import AbstractUser
from django.db import models


class User(AbstractUser):
    pass


class ProgramCode(models.Model):
    id = models.UUIDField(primary_key=True, default=uuid.uuid4, editable=False)
    title = models.CharField(max_length=255, default="Untitled")
    code = models.TextField()
    created_at = models.DateTimeField(auto_now_add=True)
    user = models.ForeignKey(User, null=True, on_delete=models.CASCADE)

    def __str__(self) -> str:
        return f"{self.code} - {self.user}"
