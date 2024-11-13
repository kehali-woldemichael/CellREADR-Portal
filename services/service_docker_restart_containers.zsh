#!/bin/zsh


function main() {
  restart_container frontend
  restart_container ensembl
}

function restart_container() {
  container_name="$1"
  # Step 1 check running containers 
  status_frontend=$(docker inspect -f '{{.State.Status}}' "$container_name")
  

  if [[ "$status_frontend" = "exited" ]]; then
    echo "restarting container $container_name"
    docker container start $container_name
    echo "\n"
  else
    echo "$container_name is not exited"
    echo "\n"
  fi 

}

main
