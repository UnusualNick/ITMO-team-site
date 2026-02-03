from django.contrib import admin
from main import models


@admin.register(models.Player)
class PlayerAdmin(admin.ModelAdmin):
    list_display = ("name", "tg_username")

@admin.register(models.Achievment)
class AchievmentAdmin(admin.ModelAdmin):
    list_display = ("event", "rating", "link")
    list_display_links = ("event",)

