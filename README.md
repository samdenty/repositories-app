# repositories-app

A mac app which allows you to access any file from github, without cloning.

Uses a osxfuse to create a virtual GitHub folder in your home directory.
Inside it, you'll find folders of the organizations you belong to along with your username.
`~/GitHub/username/repo/README.md` for example.

The folders all have icons that are automatically sourced from GitHub.
The icons are rendered on-demand with puppeteer (resizing the browser window for each resolution).

Each icon for your repository is a React page, which you can customize in code to display a preview of the website, badges, star count etc.

## Chrome extension

A chrome extension will allow you to double click on any file on GitHub and open it instantly in Finder, VSCode etc. without cloning.
