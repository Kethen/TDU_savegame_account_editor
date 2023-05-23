if false
then
	rustup target add i686-pc-windows-gnu
fi

cargo build --target i686-pc-windows-gnu -r

export WINEPREFIX="$(pwd)/wine_prefix"

if ! [ -d "$WINEPREFIX" ]
then
	wine systeminfo
fi

wine target/i686-pc-windows-gnu/release/tdu_savegame_account_editor
