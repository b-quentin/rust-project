#!/bin/bash

# URL de votre serveur GraphQL
GRAPHQL_URL="http://127.0.0.1:8080/graphql"

# Vérifiez si un ID de champ a été passé en argument
if [ -n "$1" ]; then
    FIELD_ID=$1
    # Requête pour un champ spécifique par ID
    QUERY=$(cat <<EOF
{
  "query": "{ fields(filter: { id: \"$FIELD_ID\" }) { id collectionId valueType valueName } }"
}
EOF
)
else
    # Requête pour obtenir tous les champs
    QUERY=$(cat <<EOF
{
  "query": "{ fields { id collectionId valueType valueName } }"
}
EOF
)
fi

# Exécutez la requête curl et utilisez jq pour formater la sortie
curl -s -X POST $GRAPHQL_URL \
-H "Content-Type: application/json" \
-d "$QUERY" | jq '.data.fields'

