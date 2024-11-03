FROM postgres:15

# Set environment variables for the PostgreSQL database
# These are required to set up the initial database and user
ENV POSTGRES_HOST=localhost
ENV POSTGRES_PORT=5432
ENV POSTGRES_PASSWORD=mockall
ENV POSTGRES_USER=mockall
ENV POSTGRES_DB=mockall 

# Expose the default PostgreSQL port
EXPOSE 5433

# Start PostgreSQL
CMD ["postgres"]
