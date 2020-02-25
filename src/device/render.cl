#define OPENCL
#define OPENCL_INTEROP

#include <types.hh>
#include <random.hh>

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/hyperbolic/ray.hh>

#include <object.hh>


__kernel void render(
	__global float *screen,
	__global uchar *image,
	int width, int height,
	int sample_no,
	__global uint *seeds,
	MoebiusPk view,
	__global ObjectPk *objects,
	const int object_count
) {
	int idx = get_global_id(0);
	Rng rng;
	rand_init(&rng, seeds[idx]);

	quaternion v = q_new(
		((real)(idx % width) - 0.5f*width + rand_uniform(&rng))/height,
		((real)(idx / width) - 0.5f*height + rand_uniform(&rng))/height,
		1.0f, 0.0f
	);

	float3 color = (float3)(0.0f);

	Moebius u = mo_unpack(view);
	quaternion p = QJ;

	HyRay ray;
	ray.direction = normalize(mo_deriv(u, p, v));
	ray.start = mo_apply(u, p);

	float3 light = (float3)(1.0f);

	int prev = -1;
	for (int k = 0; k < 3; ++k) {
		int mi = -1;
		float ml = -1.0f;
		ObjectHit mcache;
		for (int i = 0; i < object_count; ++i) {
			if (prev == i) {
				continue;
			}
			Object obj = unpack_copy_object(objects[i]);
			ObjectHit cache;
			real l = object_hit(&obj, &cache, &rng, ray);
			if (l > (real)0 && (l < ml || mi < 0)) {
				mi = i;
				ml = l;
				mcache = cache;
			}
		}

		prev = mi;
		if (mi >= 0) {
			HyRay new_ray;
			Object obj = unpack_copy_object(objects[mi]);
			if (!object_bounce(&obj, &mcache, &rng, &ray, &light, &color)) {
				break;
			}
		} else {
			color += (float3)(1.0f)*light;
			break;
		}
	}

	seeds[idx] = rng.state;

	float3 avg_color = (color + vload3(idx, screen)*sample_no)/(sample_no + 1);
	vstore3(avg_color, idx, screen);

	uchar4 pix = (uchar4)(convert_uchar3(255*clamp(avg_color, 0.0f, 1.0f)), 0xff);
	vstore4(pix, idx, image);
}
