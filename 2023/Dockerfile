FROM python:3.10

WORKDIR /app

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY . /app

EXPOSE 8888
CMD [ "jupyter", "notebook", "--allow-root", "--ip=0.0.0.0", "--no-browser", "--port=8888" ]

