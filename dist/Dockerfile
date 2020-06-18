FROM alpine
WORKDIR /app
COPY ./backend /app/
ENV LISTEN 0.0.0.0:3000
EXPOSE 3000
CMD ["/bin/sh", "-c", "./backend"]
