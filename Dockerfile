FROM japaric/x86_64-pc-windows-gnu:latest

RUN apt-get update
RUN apt-get install -y libgtk-3-dev
