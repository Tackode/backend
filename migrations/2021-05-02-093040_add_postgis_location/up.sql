CREATE EXTENSION IF NOT EXISTS postgis;


ALTER TABLE "public"."place" ADD COLUMN "location" geometry(Point,4326) DEFAULT NULL;


ALTER TABLE "public"."place"
DROP COLUMN "latitude";


ALTER TABLE "public"."place"
DROP COLUMN "longitude";

