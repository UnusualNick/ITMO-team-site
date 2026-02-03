from django.contrib import admin
from django.urls import path, include
from django.conf import settings


urlpatterns = [
    path('itmoteamlk/', admin.site.urls),
    path("", include("main.urls")),
]

if settings.DEBUG:
    from django.conf.urls import static
    from django.contrib.staticfiles.urls import staticfiles_urlpatterns
    
    if settings.MEDIA_ROOT:
        urlpatterns += static.static(
            prefix=settings.MEDIA_URL,
            document_root=settings.MEDIA_ROOT,
        )
    
    urlpatterns += staticfiles_urlpatterns()
