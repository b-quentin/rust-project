#!/bin/bash

# URL de votre serveur GraphQL
GRAPHQL_URL="http://127.0.0.1:8080/graphql"

# Vérifiez si un ID a été passé en argument
if [ -n "$1" ]; then
    COLLECTION_ID=$1
    # Requête pour une collection spécifique par ID
    QUERY=$(cat <<EOF
{
  "query": "{ collections(filter: { id: \"$COLLECTION_ID\" }) { id name field } }"
}
EOF
)
else
    # Requête pour obtenir toutes les collections
    QUERY=$(cat <<EOF
{
  "query": "{ collections { id name fields { id name valueType } } }"
}
EOF
)
fi

# Exécutez la requête curl et utilisez jq pour formater la sortie
curl -s -X POST $GRAPHQL_URL \
-H "Content-Type: application/json" \
-d "$QUERY" | jq
