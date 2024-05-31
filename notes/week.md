# Week Long

Now notes will be from the next book in the series.

## Movement Blur (1)

Essentially have each ray also contain a time variable as we want to get information from the scene at a given point in time and our method of getting information is via rays. This eans we have to change up the sphere class to contain a start and end point within the scene. This becomes blur because it overlaps the snapshots of time. Time acts mostly as a dummy variable except to change the location of the spheres, which makes for pretty easy code and a nice life.

## Bounded Volume Hierarchies (2)

This is a harder section, because it is the first real optimization that we are doing. Right now this is linear time to check if a ray intersects something, but this is probably not that good because we really want to scale up the # of rays for better image quality. So, we would much rather prefer something on the order of logarithmic.

The idea comes in through having bounded volumes. Consider a tree. The root node is a bounding box of all objects in the scene. We check if the ray hits this. If it does, we consider the root nodes children. These children represent bounding boxes of subsets of the objects (where being a child implies you are a subset of the parent). Given that your ratio of objects from each depth can be about 1/2, this becomes logarithmic time to actually find the object(s) that the ray intersects.

This is idealized. Normally, we do not have an optimal ratio, and bounding boxes across children can overlap in the scene. Also, if objects move in time, it is not necessarily clear how we should make our boxes (have bounding boxes across time? have them consider the start and end positions?).

We will start with the simplest thing possible and then go from there.
