import { motion } from "framer-motion";
import { Zap, Shield, Rocket, Users } from "lucide-react";
import GridPattern from "./GridPattern";

const FEATURES = [
  {
    title: "Streaming Insights",
    desc: "SSE-powered answers & TTS audio in ≤ 60 ms.",
    icon: Zap,
  },
  {
    title: "Enterprise-grade Security",
    desc: "End-to-end encryption & RBAC across spaces.",
    icon: Shield,
  },
  {
    title: "Edge Deployment",
    desc: "Runs on Vercel Edge or anywhere via Docker.",
    icon: Rocket,
  },
  {
    title: "Team Spaces",
    desc: "Share documents & chat context with your crew.",
    icon: Users,
  },
];

export default function FeatureCards() {
  return (
    <section className="relative py-20 lg:py-32">
      <GridPattern width={22} height={22} />
      <div className="relative z-10 container grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
        {FEATURES.map((f, i) => (
          <motion.div
            key={f.title}
            initial={{ opacity: 0, y: 30 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.4, delay: i * 0.1 }}
            className="rounded-3xl p-6 bg-gradient-to-b from-background/40 to-background/60 backdrop-blur-md border border-border shadow-lg"
          >
            <f.icon className="h-8 w-8 text-primary mb-4" />
            <h3 className="font-semibold text-lg mb-2 leading-tight">{f.title}</h3>
            <p className="text-sm text-muted-foreground leading-relaxed">
              {f.desc}
            </p>
          </motion.div>
        ))}
      </div>
    </section>
  );
} 