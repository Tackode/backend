CREATE TYPE gauge_level AS ENUM ('alert', 'warning', 'safe', 'unknown');


ALTER TABLE "public"."place" ADD COLUMN "current_gauge_level" gauge_level DEFAULT 'unknown' NOT NULL;


ALTER TABLE "public"."place" ADD COLUMN "current_gauge_percent" int8 DEFAULT NULL;

