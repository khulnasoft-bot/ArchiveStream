import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "ArchiveStream | Open Web Archive",
  description: "An open-source, self-hostable web archive system",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased">{children}</body>
    </html>
  );
}
