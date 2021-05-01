ALTER TABLE "public"."checkin" ADD COLUMN "number" int8 NOT NULL DEFAULT 1;

ALTER TABLE "public"."checkin" ALTER COLUMN "duration" SET DATA TYPE int8;

ALTER TABLE "public"."place" ALTER COLUMN "average_duration" SET DATA TYPE int8;

ALTER TABLE "public"."place" ALTER COLUMN "maximum_gauge" SET DATA TYPE int8;

ALTER TABLE "public"."place" ALTER COLUMN "maximum_duration" SET DATA TYPE int8;

ALTER TABLE "public"."place" ALTER COLUMN "current_gauge" SET DATA TYPE int8;
