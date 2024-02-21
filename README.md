# Project Zomboid Discord RCON bot

Intended for use in kubernetes
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: bot-config
data:
  config.json: |
    {
        "token": "DISCORD BOT TOKEN",
        "admin_role_id": ADMIN ROLE ID FOR EXECUTING RCON COMMANDS,
        "player_log_channel_id": CHANNEL ID FOR PLAYER JOIN / LEAVE MESSAGES,
        "rcon": {
            "address": "SERVER ADRESS",
            "port": "27015",
            "password": "RCON PASSWORD"
        }
    }
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: discord-bot-extension
  namespace: project-zomboid
spec:
  replicas: 1
  selector:
    matchLabels:
      app: discord-bot-extension
  template:
    metadata:
      labels:
        app: discord-bot-extension
    spec:
      containers:
      - name: discord-bot-container
        image: maticzpl/discord-pz-bot
        volumeMounts:
        - name: config-volume
          mountPath: /config.json
          subPath: config.json
      volumes:
      - name: config-volume
        configMap:
          name: bot-config
```

Introduces /execute command  
Logs players leaving and joining to chat  
Works alongside built in project zomboid discord bot, can be used with same token.  
