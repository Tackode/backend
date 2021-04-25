ALTER TABLE "public"."checkin" ADD COLUMN "number" int4 NOT NULL DEFAULT 1;

ALTER TABLE "public"."place" DROP COLUMN "current_gauge";
