ALTER TABLE "public"."place"
DROP COLUMN "location";


DROP EXTENSION postgis;


ALTER TABLE "public"."place" ADD COLUMN "latitude" float8 DEFAULT NULL;


ALTER TABLE "public"."place" ADD COLUMN "longitude" float8 DEFAULT NULL;

