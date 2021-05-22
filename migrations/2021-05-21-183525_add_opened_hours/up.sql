CREATE TABLE "public"."opening_hour_day" (
    "id" uuid NOT NULL DEFAULT gen_random_uuid(),
    "place_id" uuid NOT NULL,
    "day" int2 NOT NULL,
    "opening_time" time NOT NULL,
    "closure_time" time NOT NULL,
    "evacuation_time" time NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);

CREATE INDEX "opening_hour_day_place_id_index" ON "public"."opening_hour_day" USING BTREE ("place_id");

ALTER TABLE "public"."opening_hour_day" ADD FOREIGN KEY ("place_id") REFERENCES "public"."place" ("id") ON DELETE CASCADE;

CREATE TABLE "public"."opening_hour_date" (
    "id" uuid NOT NULL DEFAULT gen_random_uuid(),
    "place_id" uuid NOT NULL,
    "date" date NOT NULL,
    "opening_time" time,
    "closure_time" time,
    "evacuation_time" time,
    "closed" bool NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);

CREATE INDEX "opening_hour_date_place_id_index" ON "public"."opening_hour_date" USING BTREE ("place_id");

ALTER TABLE "public"."opening_hour_date" ADD FOREIGN KEY ("place_id") REFERENCES "public"."place" ("id") ON DELETE CASCADE;

CREATE TABLE "public"."opening_hour_computed" (
    "id" uuid NOT NULL DEFAULT gen_random_uuid(),
    "place_id" uuid NOT NULL,
    "opening_timestamp" timestamptz NOT NULL,
    "closing_timestamp" timestamptz NOT NULL,
    "evacuation_timestamp" timestamptz NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);

CREATE INDEX "opening_hour_computed_place_id_index" ON "public"."opening_hour_computed" USING BTREE ("place_id");

ALTER TABLE "public"."opening_hour_computed" ADD FOREIGN KEY ("place_id") REFERENCES "public"."place" ("id") ON DELETE CASCADE;

SELECT diesel_manage_updated_at('opening_hour_day');
SELECT diesel_manage_updated_at('opening_hour_date');
SELECT diesel_manage_updated_at('opening_hour_computed');

ALTER TABLE "public"."place" ADD COLUMN "timezone" text NOT NULL DEFAULT 'Europe/Paris';
