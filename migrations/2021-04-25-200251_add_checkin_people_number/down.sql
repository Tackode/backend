ALTER TABLE "public"."checkin" DROP COLUMN "number";

ALTER TABLE "public"."checkin" ALTER COLUMN "duration" SET DATA TYPE int4;

ALTER TABLE "public"."place" ALTER COLUMN "average_duration" SET DATA TYPE int4;

ALTER TABLE "public"."place" ALTER COLUMN "maximum_gauge" SET DATA TYPE int4;

ALTER TABLE "public"."place" ALTER COLUMN "maximum_duration" SET DATA TYPE int4;

ALTER TABLE "public"."place" ALTER COLUMN "current_gauge" SET DATA TYPE int4;
