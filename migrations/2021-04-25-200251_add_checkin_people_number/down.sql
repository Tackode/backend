ALTER TABLE "public"."checkin" DROP COLUMN "number";

ALTER TABLE "public"."place" ADD COLUMN "current_gauge" int4 NOT NULL DEFAULT 0;
