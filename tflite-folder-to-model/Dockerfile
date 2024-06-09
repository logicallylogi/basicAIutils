# syntax=docker/dockerfile:1

FROM python:3.8-alpine
WORKDIR /app
COPY ./src/* .
COPY requirements.txt .
RUN pip3 install -r requirements.txt
CMD [ "python3", "src/main.py"]