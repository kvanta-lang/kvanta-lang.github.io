from django.urls import path

from code_editor.views import index, program_code


urlpatterns = [
    path("", index, name="index"),
    path("<uuid:program_code_id>", program_code, name="program_code"),
]


app_name = "code_editor"
