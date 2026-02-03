from django.db import models
from sorl.thumbnail import ImageField, get_thumbnail
from django.utils.safestring import mark_safe


class Player(models.Model):
    name = models.CharField(verbose_name="Имя", max_length=255)
    tg_username = models.CharField(verbose_name="Телеграм", max_length=63)
    rank = models.CharField(verbose_name="Звание", max_length=255, default="Стажер")

    class Meta:
        verbose_name = "игрок"
        verbose_name_plural = "игроки"

    def __str__(self):
        return self.name
    
    def save(self, *args, **kwargs):
        self.tg_username = f'@{self.tg_username.lstrip("@")}'
        super(Player, self).save(*args, **kwargs)
    

class Achievment(models.Model):
    date = models.DateField(verbose_name="Дата проведения")
    event = models.CharField(verbose_name="Название соревнования", max_length=255)
    rating = models.PositiveIntegerField(verbose_name="Место")
    link = models.CharField(verbose_name="Ссылка на соревнование", max_length=255, null=True, blank=True)

    class Meta:
        verbose_name = "достижение"
        verbose_name_plural = "достижения"

    def __str__(self):
        return self.event
    