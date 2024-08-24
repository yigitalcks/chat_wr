# Use the official Nginx image as a parent image
FROM nginx:alpine

COPY static/index.html /usr/share/nginx/html/
COPY static/styles.css /usr/share/nginx/html/

# Expose port 80
EXPOSE 80

# Start Nginx when the container launches
CMD ["nginx", "-g", "daemon off;"]