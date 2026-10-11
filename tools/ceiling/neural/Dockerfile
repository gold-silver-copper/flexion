# The neural baseline, CPU only. Build from this directory:
#   docker build -t flexion-neural tools/ceiling/neural
FROM python:3.14-slim
ENV PYTHONDONTWRITEBYTECODE=1 PYTHONUNBUFFERED=1 PIP_NO_CACHE_DIR=1
COPY requirements.txt /app/requirements.txt
RUN pip install -r /app/requirements.txt
COPY seq2seq.py /app/seq2seq.py
WORKDIR /app
USER 65534:65534
ENTRYPOINT ["python", "/app/seq2seq.py"]
