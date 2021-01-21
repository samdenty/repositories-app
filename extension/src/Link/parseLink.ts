import * as reservedNames from "github-reserved-names";

export interface User {
  type: "User";
  user: string;
}

export interface Repo {
  type: "Repo";
  user: string;
  repo: string;
}

export interface File {
  type: "File";
  user: string;
  repo: string;
  tree: string;
  path: string;
}

export interface Folder {
  type: "Folder";
  user: string;
  repo: string;
  tree: string;
  path?: string;
}

export type LinkData = User | Repo | Folder | File;
export type FolderLike = User | Repo | Folder;

reservedNames.all.push("codespaces");

export function parseLink(element: HTMLAnchorElement): LinkData | undefined {
  const url = new URL(element.href);
  if (url.hostname !== "github.com") return;
  if (url.hash) return;

  const result = url.pathname.match(
    /^\/(?<user>[^/]+)(?:\/(?:(?<repo>[^/]+)(?:\/(?:(?<type>blob|tree)\/(?<tree>[^/]+)(?:\/(?<path>.*))?)?)?)?)?$/
  )?.groups as
    | {
        user: string;
        repo?: string;
        type?: "blob" | "tree";
        tree?: string;
        path?: string;
      }
    | undefined;
  if (!result) return;

  const { user } = result;
  if (reservedNames.check(user)) return;
  if (!result.repo) return { type: "User", user };

  const { repo } = result;
  if (!result.type) return { type: "Repo", user, repo };

  if (result.type === "blob") {
    if (!result.path) return;

    return {
      type: "File",
      user,
      repo,
      tree: result.tree!,
      path: result.path,
    };
  }

  return {
    type: "Folder",
    user,
    repo,
    tree: result.tree!,
    path: result.path,
  };
}
