from django.contrib.staticfiles.storage import staticfiles_storage
from django.urls import path
from django.views.generic import RedirectView

from code_editor.views import index, program_code, update_title


urlpatterns = [
    path("", index, name="index"),
    path("<uuid:program_code_id>", program_code, name="program_code"),
    path("update_title/<uuid:program_code_id>", update_title, name="update_title"),
    path('favicon.ico', RedirectView.as_view(url=staticfiles_storage.url('img/favicon.ico')))

]


app_name = "code_editor"
