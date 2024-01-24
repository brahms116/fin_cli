set -e

set -a
. ./.env
set +a

cargo run -p fin_cli -- $@
