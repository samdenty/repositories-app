# repositories-app

**This project is super WIP, and not ready for usage yet.**

A Mac app which allows you to access any file / folder on github, without cloning.

Creates a virtual `GitHub` folder in your home directory. Files from repos can be accessed using paths like `~/GitHub/username/repo/README.md`. When filesystem operations are performed, the results are cached and persisted to a local database.

<a href="https://www.youtube.com/watch?v=gNhPOx4m19M">
  <img src="https://yt-embed.herokuapp.com/embed?v=gNhPOx4m19M" width="500" alt="Prototype">
</a>

All user folders have icons sourced from GitHub:

![User icons](./screenshots/User%20icons.png)

Repo folders have icons sourced from automatically crawling the linked website / README:

![Pathbar](./screenshots/Pathbar.png)

And all files use the same icon pack that you use in your IDE:

![File icons](./screenshots/File%20icons.png)

## Chrome extension

A chrome extension allows you to right click on any file on GitHub and open it instantly in Finder, VSCode etc. without cloning.

![Context menus](./screenshots/Context%20menus.png)
