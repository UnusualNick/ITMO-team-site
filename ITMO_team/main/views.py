from django.shortcuts import render
from django.http import HttpResponse
from main.models import Player, Achievment


def home_page(request):
    return render(
        request=request,
        template_name="main/home.html",
    )


def achievements(request):
    _achievements = Achievment.objects.all().order_by("-date")
    return render(
        request=request, 
        template_name="main/achievements.html",
        context={
            "achievements": _achievements,
        },
    )


def players(request):
    _players = Player.objects.all()
    return render(
        request=request, 
        template_name="main/players.html",
        context={
            "players": _players,
        },
    )


def feedback(request):
    return HttpResponse("feedback")
