
#!/bin/bash

# Vérifiez si les arguments nécessaires ont été fournis
if [ $# -ne 3 ]; then
  echo "Usage: $0 <collection_id> <value_type> <value_name>"
  exit 1
fi

# Arguments
COLLECTION_ID=$1
VALUE_TYPE=$2
VALUE_NAME=$3

# URL de votre serveur GraphQL
GRAPHQL_URL="http://127.0.0.1:8080/graphql"

# Requête pour créer un nouveau champ
QUERY=$(cat <<EOF
{
  "query": "mutation { createField(collectionId: \"$COLLECTION_ID\", valueType: \"$VALUE_TYPE\", name: \"$VALUE_NAME\") { id collectionId valueType name } }"
}
EOF
)

# Exécutez la requête curl et utilisez jq pour formater la sortie
curl -s -X POST $GRAPHQL_URL \
-H "Content-Type: application/json" \
-d "$QUERY" | jq '.data.createField'
