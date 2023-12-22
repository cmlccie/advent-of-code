.PHONY: clean

clean:
	find . -name "__pycache__" -type d -exec rm -r {} +
	find . -name ".pytest_cache" -type d -exec rm -r {} +
	find . -name "*.pyc" -delete
	find . -name ".target" -type d -exec rm -r {} +
	find . -name ".build" -type d -exec rm -r {} +
