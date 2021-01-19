import * as reservedNames from "github-reserved-names";

reservedNames.all.push("codespaces");

export enum LinkType {
  User = "User",
  Repo = "Repo",
  File = "File",
  Folder = "Folder",
}

export interface User {
  type: LinkType.User;
  user: string;
}

export interface Repo {
  type: LinkType.Repo;
  repo: string;
}

export interface File {
  type: LinkType.File;
  user: string;
  repo: string;
  ref: string;
  path: string;
}

export interface Folder {
  type: LinkType.Folder;
  user: string;
  repo: string;
  ref: string;
  path?: string;
}

export type LinkData = User | Repo | Folder | File;

export function parseLink(element: HTMLAnchorElement) {
  const url = new URL(element.href);
  if (url.hostname !== "github.com") return;
  if (url.hash) return;

  const result = url.pathname.match(
    /^\/(?<user>[^/]+)(?:\/(?:(?<repo>[^/]+)(?:\/(?:(?<type>blob|tree)\/(?<ref>[^/]+)(?:\/(?<path>.*))?)?)?)?)?$/
  )?.groups as
    | {
        user: string;
        repo?: string;
        type?: "blob" | "tree";
        ref?: string;
        path?: string;
      }
    | undefined;
  if (!result) return;

  const { user } = result;
  if (reservedNames.check(user)) return;
  if (!result.repo) return { type: LinkType.User, user };

  const { repo } = result;
  if (!result.type) return { type: LinkType.Repo, user, repo };

  if (result.type === "blob" && !result.path) return;

  return {
    type: result.type === "blob" ? LinkType.File : LinkType.Folder,
    user,
    repo,
    ref: result.ref,
    path: result.path,
  };
}
