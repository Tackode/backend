ALTER TABLE "public"."place" ADD COLUMN "maximum_gauge" int4 DEFAULT NULL;

ALTER TABLE "public"."place" ADD COLUMN "current_gauge" int4 NOT NULL DEFAULT 0;

ALTER TABLE "public"."place" ADD COLUMN "address" text DEFAULT NULL;

ALTER TABLE "public"."place" ADD COLUMN "latitude" float8 DEFAULT NULL;

ALTER TABLE "public"."place" ADD COLUMN "longitude" float8 DEFAULT NULL;

ALTER TABLE "public"."place" ADD COLUMN "maximum_duration" int4 NOT NULL DEFAULT 300;

ALTER TABLE "public"."checkin" ALTER COLUMN "duration" SET DATA TYPE int4;

ALTER TABLE "public"."place" ALTER COLUMN "average_duration" SET DATA TYPE int4;
