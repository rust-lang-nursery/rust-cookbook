# Returns 1 if program is installed and 0 otherwise
function program_installed {
    local return_=1

    type $1 >/dev/null 2>&1 || { local return_=0; }

    echo "$return_"
}

function main() {
    # Ensure required programs are installed
    if [ $(program_installed git) == 0 ]; then
        echo "Please install Git."
    elif [ $(program_installed ghp-import) == 0 ]; then
        echo "Please install ghp-import: pip install ghp-import."
    elif [ $(program_installed mdbook) == 0 ]; then
        echo "Please install mdbook: cargo install mdbook."
    else
        echo "Copying CONTRIBUTING.md to src/pages/contributing.md"
        cp CONTRIBUTING.md src/pages/contributing.md
        echo "Building site to book/"
        mdbook build
        echo "Committing book directory to gh-pages branch"
        ghp-import -p book
        # echo ""
        # echo "To push changes to site, run ghp-import -p book"
    fi
}

main
