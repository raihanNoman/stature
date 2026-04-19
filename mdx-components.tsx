import React, { ComponentPropsWithoutRef } from "react";
import Link from "next/link";

type HeadingProps = ComponentPropsWithoutRef<"h1">;
type ParagraphProps = ComponentPropsWithoutRef<"p">;
type ListProps = ComponentPropsWithoutRef<"ul">;
type ListItemProps = ComponentPropsWithoutRef<"li">;
type AnchorProps = ComponentPropsWithoutRef<"a">;
type BlockquoteProps = ComponentPropsWithoutRef<"blockquote">;


const components = {

  // 1. Map the standard table tag
  table: (props: ComponentPropsWithoutRef<"table">) => (
    <div className="overflow-x-auto my-6">
      <table className="w-full border-collapse border border-gray-300 dark:border-zinc-600 text-left" {...props} />
    </div>
  ),

  // 2. Map the header
  thead: (props: ComponentPropsWithoutRef<"thead">) => (
    <thead className="bg-gray-100 dark:bg-zinc-700 font-medium" {...props} />
  ),

  // 3. Map table cells
  th: (props: ComponentPropsWithoutRef<"th">) => (
    <th className="border border-gray-300 dark:border-zinc-600 px-4 py-2" {...props} />
  ),
  td: (props: ComponentPropsWithoutRef<"td">) => (
    <td className="border border-gray-200 dark:border-zinc-600 px-4 py-2" {...props} />
  ),

  // 4. Map rows (optional: keep your zebra striping)
  tr: (props: ComponentPropsWithoutRef<"tr">) => (
    <tr className="even:bg-gray-50 odd:bg-white dark:even:bg-zinc-800/50 dark:odd:bg-transparent" {...props} />
  ),
  
  // Wrap everything in a "prose container"
  wrapper: ({ children }: { children: React.ReactNode }) => (
    <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8 prose prose-gray dark:prose-invert">
      {children}
    </div>
  ),

  h1: (props: HeadingProps) => (
    <h1 className="text-3xl md:text-4xl font-semibold mt-12 mb-6 leading-tight" {...props} />
  ),
  h2: (props: HeadingProps) => (
    <h2 className="text-2xl md:text-3xl font-semibold mt-10 mb-4" {...props} />
  ),
  h3: (props: HeadingProps) => (
    <h3 className="text-xl md:text-2xl font-semibold mt-8 mb-3" {...props} />
  ),
  h4: (props: HeadingProps) => (
    <h4 className="text-lg md:text-xl font-semibold mt-6 mb-2" {...props} />
  ),
  p: (props: ParagraphProps) => (
    <p className="text-base md:text-lg leading-relaxed mb-4" {...props} />
  ),
  ol: (props: ListProps) => <ol className="list-decimal pl-6 space-y-2" {...props} />,
  ul: (props: ListProps) => <ul className="list-disc pl-6 space-y-2" {...props} />,
  li: (props: ListItemProps) => <li className="mb-1" {...props} />,
  em: (props: ComponentPropsWithoutRef<"em">) => <em className="italic font-medium" {...props} />,
  strong: (props: ComponentPropsWithoutRef<"strong">) => (
    <strong className="font-semibold" {...props} />
  ),
  a: ({ href, children, ...props }: AnchorProps) => {
    const className =
      "text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-200 underline underline-offset-2";
    if (href?.startsWith("/")) {
      return (
        <Link href={href} className={className} {...props}>
          {children}
        </Link>
      );
    }
    if (href?.startsWith("#")) {
      return (
        <a href={href} className={className} {...props}>
          {children}
        </a>
      );
    }
    return (
      <a href={href} target="_blank" rel="noopener noreferrer" className={className} {...props}>
        {children}
      </a>
    );
  },
  Table: ({ data }: { data: { headers: string[]; rows: string[][] } }) => (
    <div className="overflow-x-auto my-6">
      <table className="w-full border-collapse border border-gray-300 dark:border-zinc-600 text-left">
        <thead className="bg-gray-100 dark:bg-zinc-700">
          <tr>
            {data.headers.map((header, index) => (
              <th
                key={index}
                className="border-b border-gray-300 dark:border-zinc-600 px-4 py-2 font-medium"
              >
                {header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.rows.map((row, index) => (
            <tr
              key={index}
              className="even:bg-gray-50 odd:bg-white dark:even:bg-zinc-800 dark:odd:bg-zinc-700"
            >
              {row.map((cell, cellIndex) => (
                <td
                  key={cellIndex}
                  className="border-b border-gray-200 dark:border-zinc-600 px-4 py-2"
                >
                  {cell}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  ),
  blockquote: (props: BlockquoteProps) => (
    <blockquote
      className="border-l-4 border-gray-300 dark:border-zinc-600 pl-4 italic my-6"
      {...props}
    />
  ),
};

declare global {
  type MDXProvidedComponents = typeof components;
}

export function useMDXComponents(): MDXProvidedComponents {
  return components;
}
