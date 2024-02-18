# Discord RCON bot
## WORK IN PROGRESS! Don't use!

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
        "admin_role_id": DISCORD ROLE ID FOR PERMISSION TO USE,
        "rcon": {
            "address": "SERVICE ADDRESS",
            "port": "27015",
            "password": "PASSWORD"
        }
    }
---
apiVersion: v1
kind: Pod
metadata:
  name: discord-rcon-bot
  namespace: project-zomboid
spec:
  containers:
  - name: discord-rcon-bot-container
    image: maticzpl/discord-rcon-bot
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
Works alongside built in project zomboid discord bot, can be used with same token.
