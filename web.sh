#!/usr/bin/env bash
set -euo pipefail

usage="bevy web builder
usage: $0 [-r|--release] <task>
        <task>: one of zip/serve/open
        -r, --release: optimise the build (off by default)"

release="no"
task=""

while [[ $# -gt 0 ]]; do
	case "$1" in
	-r | --release)
		release="yes"
		shift
		;;
	zip | serve | open)
		if [[ -n "$task" ]]; then
			echo "$usage" >&2
			exit 1
		fi
		task="$1"
		shift
		;;
	*)
		echo "$usage" >&2
		exit 1
		;;
	esac
done

if [[ -z "$task" ]]; then
	echo "$usage" >&2
	exit 1
fi

# check that jq and the bevy CLI are installed
if ! command -v jq &>/dev/null; then
	echo "jq could not be found, please install it" >&2
	exit 1
fi

if ! command -v bevy &>/dev/null; then
	echo "the bevy CLI could not be found, please install it" >&2
	exit 1
fi

package_name=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')

case "$task" in
zip)
	if [ "$release" == "yes" ]; then
		set -x
		RUSTFLAGS="-Zlocation-detail=none" bevy build --release web --bundle --wasm-opt "-Oz"
		(cd target/bevy_web/web-release/"$package_name"/ && zip -r - .) >"$package_name".zip
		{ set +x; } 2>/dev/null
	else
		set -x
		bevy build web --bundle
		(cd target/bevy_web/web-release/"$package_name"/ && zip -r - .) >"$package_name".zip
		{ set +x; } 2>/dev/null
	fi
	;;
serve)
	if [ "$release" == "yes" ]; then
		set -x
		bevy run --release web --wasm-opt "-Oz"
		{ set +x; } 2>/dev/null
	else
		set -x
		bevy run web
		{ set +x; } 2>/dev/null
	fi
	;;
open)
	if [ "$release" == "yes" ]; then
		set -x
		bevy run --release web --wasm-opt "-Oz" --open
		{ set +x; } 2>/dev/null
	else
		set -x
		bevy run web --open
		{ set +x; } 2>/dev/null
	fi
	;;
esac
