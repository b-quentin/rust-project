#!/bin/bash

# Vérifiez si un nom de collection a été fourni en argument
if [ -z "$1" ]; then
  echo "Usage: $0 <collection_name>"
  exit 1
fi

# Nom de la collection
COLLECTION_NAME=$1

# URL de votre serveur GraphQL
GRAPHQL_URL="http://127.0.0.1:8080/graphql"

# Requête pour créer une nouvelle collection
QUERY=$(cat <<EOF
{
  "query": "mutation { createCollection(name: \"$COLLECTION_NAME\") { id name } }"
}
EOF
)

# Exécutez la requête curl et utilisez jq pour formater la sortie
curl -s -X POST $GRAPHQL_URL \
-H "Content-Type: application/json" \
-d "$QUERY" | jq '.data.createCollection'
