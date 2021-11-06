drop table if exists "public"."short_links";
CREATE TABLE "public"."short_links" (
  "id" serial4,
  "url" text NOT NULL,
  PRIMARY KEY ("id"),
  UNIQUE(url)
);