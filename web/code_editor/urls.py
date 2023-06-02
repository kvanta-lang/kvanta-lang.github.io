from django.urls import path

from code_editor.views import index, program_code, update_title


urlpatterns = [
    path("", index, name="index"),
    path("<uuid:program_code_id>", program_code, name="program_code"),
    path("update_title/<uuid:program_code_id>", update_title, name="update_title"),
]


app_name = "code_editor"
