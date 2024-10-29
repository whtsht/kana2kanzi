FROM python:3.12

WORKDIR /app

RUN apt-get update && apt-get install -y \
    locales \
    fonts-noto-cjk \
    && echo "ja_JP UTF-8" > /etc/locale.gen \
    && locale-gen

ENV LANG=ja_JP.UTF-8
ENV LC_ALL=ja_JP.UTF-8

RUN pip install --upgrade pip
