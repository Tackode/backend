ALTER TABLE "public"."place" DROP COLUMN "maximum_gauge";

ALTER TABLE "public"."place" DROP COLUMN "current_gauge";

ALTER TABLE "public"."place" DROP COLUMN "address";

ALTER TABLE "public"."place" DROP COLUMN "latitude";

ALTER TABLE "public"."place" DROP COLUMN "longitude";

ALTER TABLE "public"."place" DROP COLUMN "maximum_duration";

ALTER TABLE "public"."checkin" ALTER COLUMN "duration" SET DATA TYPE int8;

ALTER TABLE "public"."place" ALTER COLUMN "average_duration" SET DATA TYPE int8;
