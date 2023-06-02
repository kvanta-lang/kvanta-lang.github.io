import json

from django.http import HttpResponseRedirect, HttpResponse
from django.shortcuts import render
from django.urls import reverse_lazy

from code_editor.models import ProgramCode


def index(request):
    code = ProgramCode.objects.create(
        code="setFigureColor(Color::Green);\ncircle(300, 210, 50);\n"
    )
    return HttpResponseRedirect(reverse_lazy("code_editor:program_code", args=[code.id]))


def program_code(request, program_code_id):
    if request.method == "POST":
        code = ProgramCode.objects.get(id=program_code_id)
        code.code = request.POST.get("code")
        code.save()
        return HttpResponse("Ok")
    code = ProgramCode.objects.get(id=program_code_id)
    return render(
        request,
        "index.html",
        {
            "program_code": json.dumps(code.code),
            "program_code_id": program_code_id,
            "program_code_title": code.title,
        }
    )


def update_code(request, program_code_id):
    code = ProgramCode.objects.get(id=program_code_id)
    code.code = request.POST.get("code")
    code.save()
    return HttpResponse("Ok")


def update_title(request, program_code_id):
    code = ProgramCode.objects.get(id=program_code_id)
    code.title = request.POST.get("title")
    code.save()
    return HttpResponseRedirect(reverse_lazy("code_editor:program_code", args=[code.id]))