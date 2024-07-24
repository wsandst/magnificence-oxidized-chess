cd "$(dirname "$0")"
npm run build
mkdir ./dist2
mv ./dist ./dist2/chess
rm -rf ./dist
mv ./dist2 dist