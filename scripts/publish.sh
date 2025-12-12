git add .
git commit -m "publish"
git push -u origin main
ssh quinn@serverus "cd /home/quinn/projects/amtrack_rs; git fetch; git pull; docker build . -t amtrack_rs; docker save amtrack_rs > builds/amtrack_rs.tar; scp builds/amtrack_rs.tar root@100.87.34.34:/root/docker/nginx/sites/amtrack_rs/builds"
