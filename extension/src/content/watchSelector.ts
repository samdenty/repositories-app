type Split<
  S extends string,
  D extends string
> = S extends `${infer T}${D}${infer U}` ? [T, ...Split<U, D>] : [S];
type TakeLast<V> = V extends []
  ? never
  : V extends [string]
  ? V[0]
  : V extends [string, ...infer R]
  ? TakeLast<R>
  : never;
type TrimLeft<V extends string> = V extends ` ${infer R}` ? TrimLeft<R> : V;
type TrimRight<V extends string> = V extends `${infer R} ` ? TrimRight<R> : V;
type Trim<V extends string> = TrimLeft<TrimRight<V>>;
type StripModifier<
  V extends string,
  M extends string
> = V extends `${infer L}${M}${infer A}` ? L : V;
type StripModifiers<V extends string> = StripModifier<
  StripModifier<StripModifier<StripModifier<V, ".">, "#">, "[">,
  ":"
>;
type TakeLastAfterToken<V extends string, T extends string> = StripModifiers<
  TakeLast<Split<Trim<V>, T>>
>;
type GetLastElementName<V extends string> = TakeLastAfterToken<
  TakeLastAfterToken<V, " ">,
  ">"
>;
type GetEachElementName<V, L extends string[] = []> = V extends []
  ? L
  : V extends [string]
  ? [...L, GetLastElementName<V[0]>]
  : V extends [string, ...infer R]
  ? GetEachElementName<R, [...L, GetLastElementName<V[0]>]>
  : [];
type GetElementNames<V extends string> = GetEachElementName<Split<V, ",">>;
type ElementByName<V extends string> = V extends keyof HTMLElementTagNameMap
  ? HTMLElementTagNameMap[V]
  : V extends keyof SVGElementTagNameMap
  ? SVGElementTagNameMap[V]
  : Element;
type MatchEachElement<V, L extends Element | null = null> = V extends []
  ? L
  : V extends [string]
  ? L | ElementByName<V[0]>
  : V extends [string, ...infer R]
  ? MatchEachElement<R, L | ElementByName<V[0]>>
  : L;

type QueryResult<T extends string> = MatchEachElement<GetElementNames<T>>;

export function watchSelector<T extends string>(
  selector: T,
  callback: (element: NonNullable<QueryResult<T>>) => void
) {
  document.querySelectorAll(selector).forEach(callback as any);

  const observer = new MutationObserver((mutations) => {
    const addedNodes = mutations
      .map((mutation) => Array.from(mutation.addedNodes))
      .flat();

    for (const node of addedNodes) {
      if (node.nodeType !== Node.ELEMENT_NODE) continue;
      const element = node as HTMLElement;
      element.querySelectorAll(selector).forEach(callback as any);

      if (element.matches(selector)) {
        callback(element as any);
      }
    }
  });

  // Set up observer.
  observer.observe(document.documentElement, {
    childList: true,
    subtree: true,
  });

  return () => observer.disconnect();
}
