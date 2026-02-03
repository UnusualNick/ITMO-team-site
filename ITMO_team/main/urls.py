from django.urls import path
from main import views

urlpatterns = [
    path('home/', views.home_page, name="home"),
    path('achievements/', views.achievements, name="achievements"),
    path('players/', views.players, name="players"),
    path('feedback/', views.feedback, name="feedback"),
    path('', views.home_page),

]
