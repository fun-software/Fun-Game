import { createClient } from "@supabase/supabase-js";

let supabaseURL = process.env.SUPABASE_PROJECT_URL;
let supabaseKey = process.env.SUPABASE_API_KEY;

export const supabase = createClient(supabaseURL, supabaseKey);
