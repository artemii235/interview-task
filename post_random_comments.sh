#!/bin/bash

# Script to post 1000 random comments to random topics

# Configuration
API_URL="http://localhost:8080/test-task/comments"
NUM_REQUESTS=10000

# Arrays for random data generation
SENDERS=("Alice" "Bob" "Charlie" "Dave" "Eve" "Frank" "Grace" "Heidi" "Ivan" "Judy")
COMMENT_PREFIXES=("I think" "In my opinion" "I believe" "I wonder if" "Maybe" "Perhaps" "Definitely" "I disagree that" "I agree that" "It seems like")
COMMENT_SUBJECTS=("this topic" "the previous comment" "the original post" "your idea" "that concept" "this discussion" "your question" "this issue" "the solution" "this problem")
COMMENT_SUFFIXES=("is interesting." "needs more thought." "is brilliant!" "could be improved." "is not clear." "makes sense." "is confusing." "is well explained." "requires clarification." "is spot on!")

# Function to generate a random UUID
generate_uuid() {
    if command -v uuidgen &> /dev/null; then
        uuidgen
    else
        # Fallback method if uuidgen is not available
        python3 -c "import uuid; print(uuid.uuid4())"
    fi
}

# Array to store topic IDs for reuse
TOPIC_IDS=()

# Function to generate a random comment
generate_comment() {
    local topic_id=$1
    local sender=${SENDERS[$RANDOM % ${#SENDERS[@]}]}
    local prefix=${COMMENT_PREFIXES[$RANDOM % ${#COMMENT_PREFIXES[@]}]}
    local subject=${COMMENT_SUBJECTS[$RANDOM % ${#COMMENT_SUBJECTS[@]}]}
    local suffix=${COMMENT_SUFFIXES[$RANDOM % ${#COMMENT_SUFFIXES[@]}]}

    local text="$prefix $subject $suffix"

    echo "{\"topic_id\":\"$topic_id\",\"sender\":\"$sender\",\"text\":\"$text\"}"
}

# Main loop to post comments
for ((i=1; i<=$NUM_REQUESTS; i++)); do
    # Decide whether to reuse an existing topic ID or generate a new one
    if [ ${#TOPIC_IDS[@]} -gt 0 ] && [ $((RANDOM % 3)) -eq 0 ]; then
        # Reuse a random topic ID from the array (1/3 probability)
        topic_id=${TOPIC_IDS[$RANDOM % ${#TOPIC_IDS[@]}]}
        echo "Reusing topic ID: $topic_id"
    else
        # Generate a new topic ID
        topic_id=$(generate_uuid)
        # Add it to the array for potential future reuse
        TOPIC_IDS+=("$topic_id")
        echo "Generated new topic ID: $topic_id"
    fi

    comment_data=$(generate_comment "$topic_id")

    echo "Posting comment $i of $NUM_REQUESTS:"
    echo "$comment_data"

    response=$(curl -s -X POST "$API_URL" \
        -H "Content-Type: application/json" \
        -d "$comment_data")

    echo "Response: $response"
    echo "------------------------------------"

    sleep 0.001
done

echo "Completed sending $NUM_REQUESTS comments."
